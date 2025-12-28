macro_rules! deps {
    () => {
        ErrorKind!();
    };
}

macro_rules! impl_404 {
    () => {
        deps!();
        impl ErrorKind { # [doc = " End-user description of the error case, where relevant"] pub fn as_str (self) -> Option < & 'static str > { match self { Self :: InvalidValue => Some ("one of the values isn't valid for an argument") , Self :: UnknownArgument => Some ("unexpected argument found") , Self :: InvalidSubcommand => Some ("unrecognized subcommand") , Self :: NoEquals => Some ("equal is needed when assigning values to one of the arguments") , Self :: ValueValidation => Some ("invalid value for one of the arguments") , Self :: TooManyValues => Some ("unexpected value for an argument found") , Self :: TooFewValues => Some ("more values required for an argument") , Self :: WrongNumberOfValues => Some ("too many or too few values for an argument") , Self :: ArgumentConflict => { Some ("an argument cannot be used with one or more of the other specified arguments") } Self :: MissingRequiredArgument => { Some ("one or more required arguments were not provided") } Self :: MissingSubcommand => Some ("a subcommand is required but one was not provided") , Self :: InvalidUtf8 => Some ("invalid UTF-8 was detected in one or more arguments") , Self :: DisplayHelp => None , Self :: DisplayHelpOnMissingArgumentOrSubcommand => None , Self :: DisplayVersion => None , Self :: Io => None , Self :: Format => None , } } }
    };
}

impl_404!();