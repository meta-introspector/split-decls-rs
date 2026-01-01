# AST Trace: ../rust/compiler/rustc_ast_lowering/src/stability.rs

Generated 8 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
use std::fmt;

use rustc_abi::ExternAbi;
use rustc_feature::Features;
use rustc_session::Session;
use rustc_session::parse::feature_err;
use rustc_span::symbol::sym;
use rustc_span::{Span, Symbol};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
pub(crate) fn enabled_names(features: &rustc_feature::Features, span: Span) -> Vec<&'static str> {
    ExternAbi::ALL_VARIANTS
        .into_iter()
        .filter(|abi| extern_abi_enabled(features, span, **abi).is_ok())
        .map(|abi| abi.as_str())
        .collect()
}
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=8 | LINES=14

```rust
pub(crate) fn extern_abi_enabled(
    features: &rustc_feature::Features,
    span: Span,
    abi: ExternAbi,
) -> Result<(), UnstableAbi> {
    extern_abi_stability(abi).or_else(|unstable @ UnstableAbi { feature, .. }| {
        if features.enabled(feature) || span.allows_unstable(feature) {
            Ok(())
        } else {
            Err(unstable)
        }
    })
}
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=7 | LINES=11

```rust
#[allow(rustc::untranslatable_diagnostic)]
pub(crate) fn gate_unstable_abi(sess: &Session, features: &Features, span: Span, abi: ExternAbi) {
    match extern_abi_enabled(features, span, abi) {
        Ok(_) => (),
        Err(unstable_abi) => {
            let explain = unstable_abi.to_string();
            feature_err(sess, unstable_abi.feature, span, explain).emit();
        }
    }
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=STRUCT | NAME=UnstableAbi | COMPLEXITY=2 | LINES=6

```rust
pub struct UnstableAbi {
    abi: ExternAbi,
    feature: Symbol,
    explain: GateReason,
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=5

```rust
enum GateReason {
    Experimental,
    ImplDetail,
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=14 | LINES=14

```rust
impl fmt::Display for UnstableAbi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { abi, .. } = self;
        match self.explain {
            GateReason::Experimental => {
                write!(f, "the extern {abi} ABI is experimental and subject to change")
            }
            GateReason::ImplDetail => {
                write!(f, "the extern {abi} ABI is an implementation detail and perma-unstable")
            }
        }
    }
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=FUNCTION | NAME=extern_abi_stability | COMPLEXITY=40 | LINES=78

```rust
pub fn extern_abi_stability(abi: ExternAbi) -> Result<(), UnstableAbi> {
    match abi {
        // stable ABIs
        ExternAbi::Rust
        | ExternAbi::C { .. }
        | ExternAbi::Cdecl { .. }
        | ExternAbi::Stdcall { .. }
        | ExternAbi::Fastcall { .. }
        | ExternAbi::Thiscall { .. }
        | ExternAbi::Aapcs { .. }
        | ExternAbi::Win64 { .. }
        | ExternAbi::SysV64 { .. }
        | ExternAbi::System { .. }
        | ExternAbi::EfiApi => Ok(()),
        ExternAbi::Unadjusted => {
            Err(UnstableAbi { abi, feature: sym::abi_unadjusted, explain: GateReason::ImplDetail })
        }
        // experimental
        ExternAbi::Vectorcall { .. } => Err(UnstableAbi {
            abi,
            feature: sym::abi_vectorcall,
            explain: GateReason::Experimental,
        }),
        ExternAbi::RustCall => Err(UnstableAbi {
            abi,
            feature: sym::unboxed_closures,
            explain: GateReason::Experimental,
        }),
        ExternAbi::RustCold => {
            Err(UnstableAbi { abi, feature: sym::rust_cold_cc, explain: GateReason::Experimental })
        }
        ExternAbi::RustInvalid => {
            Err(UnstableAbi { abi, feature: sym::rustc_attrs, explain: GateReason::ImplDetail })
        }
        ExternAbi::GpuKernel => Err(UnstableAbi {
            abi,
            feature: sym::abi_gpu_kernel,
            explain: GateReason::Experimental,
        }),
        ExternAbi::PtxKernel => {
            Err(UnstableAbi { abi, feature: sym::abi_ptx, explain: GateReason::Experimental })
        }
        ExternAbi::Msp430Interrupt => Err(UnstableAbi {
            abi,
            feature: sym::abi_msp430_interrupt,
            explain: GateReason::Experimental,
        }),
        ExternAbi::X86Interrupt => Err(UnstableAbi {
            abi,
            feature: sym::abi_x86_interrupt,
            explain: GateReason::Experimental,
        }),
        ExternAbi::AvrInterrupt | ExternAbi::AvrNonBlockingInterrupt => Err(UnstableAbi {
            abi,
            feature: sym::abi_avr_interrupt,
            explain: GateReason::Experimental,
        }),
        ExternAbi::RiscvInterruptM | ExternAbi::RiscvInterruptS => Err(UnstableAbi {
            abi,
            feature: sym::abi_riscv_interrupt,
            explain: GateReason::Experimental,
        }),
        ExternAbi::CmseNonSecureCall => Err(UnstableAbi {
            abi,
            feature: sym::abi_cmse_nonsecure_call,
            explain: GateReason::Experimental,
        }),
        ExternAbi::CmseNonSecureEntry => Err(UnstableAbi {
            abi,
            feature: sym::cmse_nonsecure_entry,
            explain: GateReason::Experimental,
        }),
        ExternAbi::Custom => {
            Err(UnstableAbi { abi, feature: sym::abi_custom, explain: GateReason::Experimental })
        }
    }
}
```

---
*Generated by AST tracing system*
