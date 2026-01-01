# AST Trace: ../rust/compiler/rustc_borrowck/src/diagnostics/find_use.rs

Generated 11 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=4

```rust
use std::collections::VecDeque;

use rustc_data_structures::fx::FxIndexSet;
use rustc_middle::mir::visit::{PlaceContext, Visitor};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use rustc_middle::mir::{self, Body, Local, Location};
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use rustc_middle::ty::{RegionVid, TyCtxt};
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
use crate::def_use::{self, DefUse};
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use crate::region_infer::{Cause, RegionInferenceContext};
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=3 | LINES=12

```rust
pub(crate) fn find<'tcx>(
    body: &Body<'tcx>,
    regioncx: &RegionInferenceContext<'tcx>,
    tcx: TyCtxt<'tcx>,
    region_vid: RegionVid,
    start_point: Location,
) -> Option<Cause> {
    let mut uf = UseFinder { body, regioncx, tcx, region_vid, start_point };

    uf.find()
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=STRUCT | NAME=UseFinder | COMPLEXITY=2 | LINES=8

```rust
struct UseFinder<'a, 'tcx> {
    body: &'a Body<'tcx>,
    regioncx: &'a RegionInferenceContext<'tcx>,
    tcx: TyCtxt<'tcx>,
    region_vid: RegionVid,
    start_point: Location,
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=FUNCTION | NAME=find | COMPLEXITY=37 | LINES=66

```rust
impl<'a, 'tcx> UseFinder<'a, 'tcx> {
    fn find(&mut self) -> Option<Cause> {
        let mut queue = VecDeque::new();
        let mut visited = FxIndexSet::default();

        queue.push_back(self.start_point);
        while let Some(p) = queue.pop_front() {
            if !self.regioncx.region_contains(self.region_vid, p) {
                continue;
            }

            if !visited.insert(p) {
                continue;
            }

            let block_data = &self.body[p.block];

            let mut visitor = DefUseVisitor {
                body: self.body,
                tcx: self.tcx,
                region_vid: self.region_vid,
                def_use_result: None,
            };

            let is_statement = p.statement_index < block_data.statements.len();

            if is_statement {
                visitor.visit_statement(&block_data.statements[p.statement_index], p);
            } else {
                visitor.visit_terminator(block_data.terminator.as_ref().unwrap(), p);
            }

            match visitor.def_use_result {
                Some(DefUseResult::Def) => {}

                Some(DefUseResult::UseLive { local }) => {
                    return Some(Cause::LiveVar(local, p));
                }

                Some(DefUseResult::UseDrop { local }) => {
                    return Some(Cause::DropVar(local, p));
                }

                None => {
                    if is_statement {
                        queue.push_back(p.successor_within_block());
                    } else {
                        queue.extend(
                            block_data
                                .terminator()
                                .successors()
                                .filter(|&bb| {
                                    Some(&mir::UnwindAction::Cleanup(bb))
                                        != block_data.terminator().unwind()
                                })
                                .map(|bb| Location { statement_index: 0, block: bb }),
                        );
                    }
                }
            }
        }

        None
    }
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=STRUCT | NAME=DefUseVisitor | COMPLEXITY=2 | LINES=7

```rust
struct DefUseVisitor<'a, 'tcx> {
    body: &'a Body<'tcx>,
    tcx: TyCtxt<'tcx>,
    region_vid: RegionVid,
    def_use_result: Option<DefUseResult>,
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=4 | LINES=6

```rust
enum DefUseResult {
    Def,
    UseLive { local: Local },
    UseDrop { local: Local },
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=FUNCTION | NAME=visit_local | COMPLEXITY=19 | LINES=22

```rust
impl<'a, 'tcx> Visitor<'tcx> for DefUseVisitor<'a, 'tcx> {
    fn visit_local(&mut self, local: Local, context: PlaceContext, _: Location) {
        let local_ty = self.body.local_decls[local].ty;

        let mut found_it = false;
        self.tcx.for_each_free_region(&local_ty, |r| {
            if r.as_var() == self.region_vid {
                found_it = true;
            }
        });

        if found_it {
            self.def_use_result = match def_use::categorize(context) {
                Some(DefUse::Def) => Some(DefUseResult::Def),
                Some(DefUse::Use) => Some(DefUseResult::UseLive { local }),
                Some(DefUse::Drop) => Some(DefUseResult::UseDrop { local }),
                None => None,
            };
        }
    }
}
```

---
*Generated by AST tracing system*
