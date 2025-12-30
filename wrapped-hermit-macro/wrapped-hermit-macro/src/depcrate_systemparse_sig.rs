// Generated macro for parse_sig (function)
macro_rules! Depcrate_systemparse_sig {
() => {
// Module: crate::system
// Provides: {"parse_sig"}
// Dependencies: {}
fn parse_sig (sig : & Signature) -> Result < ParsedSig > { if let Some (constness) = sig . constness { bail ! (constness , "#[system] is not supported on const functions") ; } if let Some (asyncness) = sig . asyncness { bail ! (asyncness , "#[system] is not supported on async functions") ; } match & sig . abi { Some (Abi { extern_token : _ , name : Some (name) , }) if matches ! (&* name . value () , "C" | "C-unwind") => { } _ => bail ! (& sig . abi , "#[system] functions must be `extern \"C\"` or `extern \"C-unwind\"`") , } if ! sig . generics . params . is_empty () { bail ! (& sig . generics , "#[system] cannot be used with generic functions") ; } if ! sig . ident . to_string () . starts_with ("sys_") { bail ! (& sig . ident , "#[system] functions must start with `sys_`") ; } let mut args = vec ! [] ; for arg in & sig . inputs { let pat = match arg { syn :: FnArg :: Receiver (_) => bail ! (arg , "#[system] functions cannot take `self`") , syn :: FnArg :: Typed (pat) => pat , } ; if let Pat :: Ident (pat) = & * pat . pat { args . push (pat . ident . clone ()) ; } else { bail ! (pat , "unsupported pattern in #[system] function argument") ; } } Ok (ParsedSig { args }) }
};
}
