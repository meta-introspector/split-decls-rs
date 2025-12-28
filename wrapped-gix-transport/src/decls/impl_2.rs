macro_rules! deps {
    () => {
        Service!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl Service { # [doc = " Render this instance as string recognized by the git transport layer."] pub fn as_str (& self) -> & 'static str { match self { Service :: ReceivePack => "git-receive-pack" , Service :: UploadPack => "git-upload-pack" , } } }
    };
}

impl_2!();