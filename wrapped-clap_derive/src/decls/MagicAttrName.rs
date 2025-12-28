macro_rules! deps {
    () => {
        Action!();
        ValueParser!();
    };
}

macro_rules! MagicAttrName {
    () => {
        deps!();
        # [derive (Copy , Clone , PartialEq , Eq)] pub (crate) enum MagicAttrName { Short , Long , ValueParser , Action , Env , Flatten , ValueEnum , FromGlobal , Subcommand , VerbatimDocComment , ExternalSubcommand , About , LongAbout , LongHelp , Author , Version , RenameAllEnv , RenameAll , Skip , DefaultValueT , DefaultValuesT , DefaultValueOsT , DefaultValuesOsT , NextDisplayOrder , NextHelpHeading , }
    };
}

MagicAttrName!()