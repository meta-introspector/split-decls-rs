macro_rules! deps {
    () => {
        VersionTable!();
        FileHeader!();
    };
}

macro_rules! impl_429 {
    () => {
        deps!();
        impl < 'data , Elf : FileHeader > Default for VersionTable < 'data , Elf > { fn default () -> Self { VersionTable { symbols : & [] , versions : Vec :: new () , } } }
    };
}

impl_429!()