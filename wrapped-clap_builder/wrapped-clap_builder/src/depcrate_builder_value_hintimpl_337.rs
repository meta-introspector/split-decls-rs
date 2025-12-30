// Generated macro for impl_337 (impl)
macro_rules! Depcrate_builder_value_hintimpl_337 {
() => {
// Module: crate::builder::value_hint
// Provides: {"impl_337"}
// Dependencies: {}
impl FromStr for ValueHint { type Err = String ; fn from_str (s : & str) -> Result < Self , < Self as FromStr > :: Err > { Ok (match & * s . to_ascii_lowercase () { "unknown" => ValueHint :: Unknown , "other" => ValueHint :: Other , "anypath" => ValueHint :: AnyPath , "filepath" => ValueHint :: FilePath , "dirpath" => ValueHint :: DirPath , "executablepath" => ValueHint :: ExecutablePath , "commandname" => ValueHint :: CommandName , "commandstring" => ValueHint :: CommandString , "commandwitharguments" => ValueHint :: CommandWithArguments , "username" => ValueHint :: Username , "hostname" => ValueHint :: Hostname , "url" => ValueHint :: Url , "emailaddress" => ValueHint :: EmailAddress , _ => return Err (format ! ("unknown ValueHint: `{s}`")) , }) } }
};
}
