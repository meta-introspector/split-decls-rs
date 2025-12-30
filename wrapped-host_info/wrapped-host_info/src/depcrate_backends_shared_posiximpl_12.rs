// Generated macro for impl_12 (impl)
macro_rules! Depcrate_backends_shared_posiximpl_12 {
() => {
// Module: crate::backends::shared::posix
// Provides: {"impl_12"}
// Dependencies: {}
impl FromStr for LocaleCategory { type Err = HostInfoError ; fn from_str (s : & str) -> Result < Self , Self :: Err > { match s { "LC_CTYPE" => Ok (Self :: Character) , "LC_NUMERIC" => Ok (Self :: Number) , "LC_TIME" => Ok (Self :: Time) , "LC_COLLATE" => Ok (Self :: Collate) , "LC_MONETARY" => Ok (Self :: Monetary) , "LC_MESSAGES" => Ok (Self :: Messages) , "LC_PAPER" => Ok (Self :: Paper) , "LC_NAME" => Ok (Self :: Name) , "LC_ADDRESS" => Ok (Self :: Address) , "LC_TELEPHONE" => Ok (Self :: Telephone) , "LC_MEASUREMENT" => Ok (Self :: Measurement) , "LC_IDENTIFICATION" => Ok (Self :: Identification) , "LC_ALL" => Ok (Self :: All) , _ => Err (HostInfoError :: UnknownCategory) , } } }
};
}
