macro_rules! deps {
    () => {
        TargetDirectoryConfig!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl TargetDirectoryConfig { pub fn target_dir < 'a > (& 'a self , ws_target_dir : Option < & 'a Utf8Path > ,) -> Option < Cow < 'a , Utf8Path > > { match self { TargetDirectoryConfig :: None => None , TargetDirectoryConfig :: UseSubdirectory => { Some (Cow :: Owned (ws_target_dir ? . join ("rust-analyzer"))) } TargetDirectoryConfig :: Directory (dir) => Some (Cow :: Borrowed (dir)) , } } }
    };
}

impl_48!()