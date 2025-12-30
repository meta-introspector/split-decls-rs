// Generated macro for impl_150 (impl)
macro_rules! Depcrate_read_utilimpl_150 {
() => {
// Module: crate::read::util
// Provides: {"impl_150"}
// Dependencies: {}
impl < 'data , R : ReadRef < 'data > > StringTable < 'data , R > { # [doc = " Interpret the given data as a string table."] pub fn new (data : R , start : u64 , end : u64) -> Self { StringTable { data : Some (data) , start , end , marker : PhantomData , } } # [doc = " Return the string at the given offset."] pub fn get (& self , offset : u32) -> Result < & 'data [u8] , () > { match self . data { Some (data) => { let r_start = self . start . checked_add (offset . into ()) . ok_or (()) ? ; data . read_bytes_at_until (r_start .. self . end , 0) } None => Err (()) , } } }
};
}
