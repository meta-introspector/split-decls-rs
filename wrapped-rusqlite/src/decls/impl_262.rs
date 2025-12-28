macro_rules! deps {
    () => {
        Result!();
        Changegroup!();
        Changeset!();
    };
}

macro_rules! impl_262 {
    () => {
        deps!();
        impl Changegroup { # [doc = " Create a new change group."] # [inline] pub fn new () -> Result < Self > { let mut cg = ptr :: null_mut () ; check (unsafe { ffi :: sqlite3changegroup_new (& mut cg) }) ? ; Ok (Changegroup { cg }) } # [doc = " Add a changeset"] # [inline] pub fn add (& mut self , cs : & Changeset) -> Result < () > { check (unsafe { ffi :: sqlite3changegroup_add (self . cg , cs . n , cs . cs) }) } # [doc = " Add a changeset read from `input` to this change group."] # [inline] pub fn add_stream (& mut self , input : & mut dyn Read) -> Result < () > { let input_ref = & input ; check (unsafe { ffi :: sqlite3changegroup_add_strm (self . cg , Some (x_input) , input_ref as * const & mut dyn Read as * mut c_void ,) }) } # [doc = " Obtain a composite Changeset"] # [inline] pub fn output (& mut self) -> Result < Changeset > { let mut n = 0 ; let mut output : * mut c_void = ptr :: null_mut () ; check (unsafe { ffi :: sqlite3changegroup_output (self . cg , & mut n , & mut output) }) ? ; Ok (Changeset { cs : output , n }) } # [doc = " Write the combined set of changes to `output`."] # [inline] pub fn output_strm (& mut self , output : & mut dyn Write) -> Result < () > { let output_ref = & output ; check (unsafe { ffi :: sqlite3changegroup_output_strm (self . cg , Some (x_output) , output_ref as * const & mut dyn Write as * mut c_void ,) }) } }
    };
}

impl_262!();