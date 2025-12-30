// Generated macro for concat (function)
macro_rules! Depcrate_utilconcat {
() => {
// Module: crate::util
// Provides: {"concat"}
// Dependencies: {}
pub async fn concat (mut body : h2 :: RecvStream) -> Result < Bytes , h2 :: Error > { let mut vec = Vec :: new () ; while let Some (chunk) = body . data () . await { vec . put (chunk ?) ; } Ok (vec . into ()) }
};
}
