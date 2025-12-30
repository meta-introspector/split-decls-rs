// Generated macro for impl_get_instance_packed_len (macro)
macro_rules! Depcrate_macrosimpl_get_instance_packed_len {
() => {
// Module: crate::macros
// Provides: {"impl_get_instance_packed_len"}
// Dependencies: {}
macro_rules ! impl_get_instance_packed_len { ($ borsh : ident , $ borsh_io : ident $ (,# [$ meta : meta]) ?) => { # [doc = " Helper struct which to count how much data would be written during serialization"] # [derive (Default)] struct WriteCounter { count : usize , } impl $ borsh_io :: Write for WriteCounter { fn write (& mut self , data : & [u8]) -> Result < usize , $ borsh_io :: Error > { let amount = data . len () ; self . count += amount ; Ok (amount) } fn flush (& mut self) -> Result < () , $ borsh_io :: Error > { Ok (()) } } # [doc = " Get the packed length for the serialized form of this object instance."] # [doc = ""] # [doc = " Useful when working with instances of types that contain a variable-length"] # [doc = " sequence, such as a Vec or HashMap.  Since it is impossible to know the packed"] # [doc = " length only from the type's schema, this can be used when an instance already"] # [doc = " exists, to figure out how much space to allocate in an account."] $ (# [$ meta]) ? pub fn get_instance_packed_len < T : $ borsh :: BorshSerialize > (instance : & T) -> Result < usize , $ borsh_io :: Error > { let mut counter = WriteCounter :: default () ; instance . serialize (& mut counter) ?; Ok (counter . count) } } }
};
}
