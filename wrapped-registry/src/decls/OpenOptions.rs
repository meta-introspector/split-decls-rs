macro_rules! deps {
    () => {
        Transaction!();
        Key!();
    };
}

macro_rules! OpenOptions {
    () => {
        deps!();
        # [doc = " Options and flags used to configure how a registry key is opened."] # [derive (Debug)] pub struct OpenOptions < 'a > { parent : & 'a Key , access : u32 , create : bool , transaction : Option < & 'a Transaction > , options : u32 , }
    };
}

OpenOptions!()