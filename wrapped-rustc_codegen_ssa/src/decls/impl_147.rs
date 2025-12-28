macro_rules! deps {
    () => {
        ModuleBufferMethods!();
        SerializedModule!();
    };
}

macro_rules! impl_147 {
    () => {
        deps!();
        impl < M : ModuleBufferMethods > SerializedModule < M > { pub fn data (& self) -> & [u8] { match * self { SerializedModule :: Local (ref m) => m . data () , SerializedModule :: FromRlib (ref m) => m , SerializedModule :: FromUncompressedFile (ref m) => m , } } }
    };
}

impl_147!()