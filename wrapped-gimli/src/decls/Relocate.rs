macro_rules! deps {
    () => {
        ReaderOffset!();
        Result!();
    };
}

macro_rules! Relocate {
    () => {
        deps!();
        # [doc = " Trait for relocating addresses and offsets while reading a section."] pub trait Relocate < T : ReaderOffset = usize > { # [doc = " Relocate an address which was read from the given section offset."] fn relocate_address (& self , offset : T , value : u64) -> Result < u64 > ; # [doc = " Relocate a value which was read from the given section offset."] fn relocate_offset (& self , offset : T , value : T) -> Result < T > ; }
    };
}

Relocate!()