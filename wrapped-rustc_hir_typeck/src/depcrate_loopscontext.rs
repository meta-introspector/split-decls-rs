// Generated macro for Context (enum)
macro_rules! Depcrate_loopsContext {
() => {
// Module: crate::loops
// Provides: {"Context"}
// Dependencies: {}
# [doc = " The context in which a block is encountered."] # [derive (Clone , Copy , Debug , PartialEq)] enum Context { Normal , Fn , Loop (hir :: LoopSource) , Closure (Span) , Coroutine { coroutine_span : Span , kind : hir :: CoroutineDesugaring , source : hir :: CoroutineSource , } , UnlabeledBlock (Span) , UnlabeledIfBlock (Span) , LabeledBlock , # [doc = " E.g. The labeled block inside `['_'; 'block: { break 'block 1 + 2; }]`."] AnonConst , # [doc = " E.g. `const { ... }`."] ConstBlock , # [doc = " E.g. `#[loop_match] loop { state = 'label: { /* ... */ } }`."] LoopMatch { # [doc = " The destination pointing to the labeled block (not to the loop itself)."] labeled_block : Destination , } , }
};
}
