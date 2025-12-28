macro_rules! config_to_base_path {
    () => {
        # [test] fn config_to_base_path () { for (input , expected) in [("/Applications/Xcode.app/Contents/Developer/usr/share/git-core/gitconfig" , "/Applications/Xcode.app/Contents/Developer/usr/share/git-core" ,) , ("C:/git-sdk-64/etc/gitconfig" , "C:/git-sdk-64/etc") , (r"C:\ProgramData/Git/config" , r"C:\ProgramData/Git") , ("C:/Program Files/Git/etc/gitconfig" , "C:/Program Files/Git/etc") ,] { assert_eq ! (super :: config_to_base_path (Path :: new (input)) , Path :: new (expected)) ; } }
    };
}

config_to_base_path!()