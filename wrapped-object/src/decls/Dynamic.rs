macro_rules! deps {
    () => {
        ByteString!();
    };
}

macro_rules! Dynamic {
    () => {
        deps!();
        # [doc = " An entry in the dynamic section."] # [doc = ""] # [doc = " This corresponds to [`elf::Dyn32`] or [`elf::Dyn64`]."] # [derive (Debug , Clone , PartialEq , Eq)] pub enum Dynamic < 'data > { # [doc = " The value is an automatically generated integer."] # [doc = ""] # [doc = " Writing will fail if the value cannot be automatically generated."] Auto { # [doc = " The `d_tag` field in the dynamic entry."] # [doc = ""] # [doc = " One of the `DT_*` values."] tag : u32 , } , # [doc = " The value is an integer."] Integer { # [doc = " The `d_tag` field in the dynamic entry."] # [doc = ""] # [doc = " One of the `DT_*` values."] tag : u32 , # [doc = " The `d_val` field in the dynamic entry."] val : u64 , } , # [doc = " The value is a string."] String { # [doc = " The `d_tag` field in the dynamic entry."] # [doc = ""] # [doc = " One of the `DT_*` values."] tag : u32 , # [doc = " The string value."] # [doc = ""] # [doc = " This will be stored in the dynamic string section."] val : ByteString < 'data > , } , }
    };
}

Dynamic!()