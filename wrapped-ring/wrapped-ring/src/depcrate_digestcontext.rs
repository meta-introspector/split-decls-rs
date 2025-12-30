// Generated macro for Context (struct)
macro_rules! Depcrate_digestContext {
() => {
// Module: crate::digest
// Provides: {"Context"}
// Dependencies: {}
# [doc = " A context for multi-step (Init-Update-Finish) digest calculations."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use ring::digest;"] # [doc = ""] # [doc = " let one_shot = digest::digest(&digest::SHA384, b\"hello, world\");"] # [doc = ""] # [doc = " let mut ctx = digest::Context::new(&digest::SHA384);"] # [doc = " ctx.update(b\"hello\");"] # [doc = " ctx.update(b\", \");"] # [doc = " ctx.update(b\"world\");"] # [doc = " let multi_part = ctx.finish();"] # [doc = ""] # [doc = " assert_eq!(&one_shot.as_ref(), &multi_part.as_ref());"] # [doc = " ```"] # [derive (Clone)] pub struct Context { block : BlockContext , pending : PartialBuffer < { BlockLen :: MAX . into () } > , }
};
}
