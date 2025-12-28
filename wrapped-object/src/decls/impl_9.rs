macro_rules! deps {
    () => {
        MachO!();
        BinaryFormat!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl BinaryFormat { # [doc = " The target's native binary format for relocatable object files."] # [doc = ""] # [doc = " Defaults to `Elf` for unknown platforms."] pub fn native_object () -> BinaryFormat { if cfg ! (target_os = "windows") { BinaryFormat :: Coff } else if cfg ! (target_os = "macos") { BinaryFormat :: MachO } else { BinaryFormat :: Elf } } }
    };
}

impl_9!()