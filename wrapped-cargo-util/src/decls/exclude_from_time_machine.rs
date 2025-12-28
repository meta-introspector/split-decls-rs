macro_rules! exclude_from_time_machine {
    () => {
        # [cfg (target_os = "macos")] # [doc = " Marks files or directories as excluded from Time Machine on macOS"] fn exclude_from_time_machine (path : & Path) { use core_foundation :: base :: TCFType ; use core_foundation :: { number , string , url } ; use std :: ptr ; let is_excluded_key : Result < string :: CFString , _ > = "NSURLIsExcludedFromBackupKey" . parse () ; let path = url :: CFURL :: from_path (path , false) ; if let (Some (path) , Ok (is_excluded_key)) = (path , is_excluded_key) { unsafe { url :: CFURLSetResourcePropertyForKey (path . as_concrete_TypeRef () , is_excluded_key . as_concrete_TypeRef () , number :: kCFBooleanTrue as * const _ , ptr :: null_mut () ,) ; } } }
    };
}

exclude_from_time_machine!();