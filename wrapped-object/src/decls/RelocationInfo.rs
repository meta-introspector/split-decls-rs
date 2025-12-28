macro_rules! RelocationInfo {
    () => {
        # [derive (Debug , Clone , Copy)] pub struct RelocationInfo { # [doc = " offset in the section to what is being relocated"] pub r_address : u32 , # [doc = " symbol index if r_extern == 1 or section ordinal if r_extern == 0"] pub r_symbolnum : u32 , # [doc = " was relocated pc relative already"] pub r_pcrel : bool , # [doc = " 0=byte, 1=word, 2=long, 3=quad"] pub r_length : u8 , # [doc = " does not include value of sym referenced"] pub r_extern : bool , # [doc = " if not 0, machine specific relocation type"] pub r_type : u8 , }
    };
}

RelocationInfo!()