macro_rules! ScatteredRelocationInfo {
    () => {
        # [derive (Debug , Clone , Copy)] pub struct ScatteredRelocationInfo { # [doc = " offset in the section to what is being relocated"] pub r_address : u32 , # [doc = " if not 0, machine specific relocation type"] pub r_type : u8 , # [doc = " 0=byte, 1=word, 2=long, 3=quad"] pub r_length : u8 , # [doc = " was relocated pc relative already"] pub r_pcrel : bool , # [doc = " the value the item to be relocated is referring to (without any offset added)"] pub r_value : u32 , }
    };
}

ScatteredRelocationInfo!()