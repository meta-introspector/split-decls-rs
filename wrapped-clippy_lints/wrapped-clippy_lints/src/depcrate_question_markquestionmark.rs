// Generated macro for QuestionMark (struct)
macro_rules! Depcrate_question_markQuestionMark {
() => {
// Module: crate::question_mark
// Provides: {"QuestionMark"}
// Dependencies: {}
pub struct QuestionMark { pub (crate) msrv : Msrv , pub (crate) matches_behaviour : MatchLintBehaviour , # [doc = " Keeps track of how many try blocks we are in at any point during linting."] # [doc = " This allows us to answer the question \"are we inside of a try block\""] # [doc = " very quickly, without having to walk up the parent chain, by simply checking"] # [doc = " if it is greater than zero."] # [doc = " As for why we need this in the first place: <https://github.com/rust-lang/rust-clippy/issues/8628>"] try_block_depth_stack : Vec < u32 > , # [doc = " Keeps track of the number of inferred return type closures we are inside, to avoid problems"] # [doc = " with the `Err(x.into())` expansion being ambiguous."] inferred_ret_closure_stack : u16 , }
};
}
