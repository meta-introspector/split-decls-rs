macro_rules! sdk_name {
    () => {
        # [doc = " The canonical name of the desired SDK for a given target."] pub (super) fn sdk_name (target : & Target) -> & 'static str { match (& * target . os , & * target . env) { ("macos" , "") => "MacOSX" , ("ios" , "") => "iPhoneOS" , ("ios" , "sim") => "iPhoneSimulator" , ("ios" , "macabi") => "MacOSX" , ("tvos" , "") => "AppleTVOS" , ("tvos" , "sim") => "AppleTVSimulator" , ("visionos" , "") => "XROS" , ("visionos" , "sim") => "XRSimulator" , ("watchos" , "") => "WatchOS" , ("watchos" , "sim") => "WatchSimulator" , (os , abi) => unreachable ! ("invalid os '{os}' / abi '{abi}' combination for Apple target") , } }
    };
}

sdk_name!();