// Generated macro for Flavor (trait)
macro_rules! Depcrate_ser_flavorsFlavor {
() => {
// Module: crate::ser::flavors
// Provides: {"Flavor"}
// Dependencies: {}
# [doc = " The serialization Flavor trait"] # [doc = ""] # [doc = " This is used as the primary way to encode serialized data into some kind of buffer,"] # [doc = " or modify that data in a middleware style pattern."] # [doc = ""] # [doc = " See the module level docs for an example of how flavors are used."] pub trait Flavor { # [doc = " The `Output` type is what this storage \"resolves\" to when the serialization is complete,"] # [doc = " such as a slice or a Vec of some sort."] type Output ; # [doc = " Override this method when you want to customize processing"] # [doc = " multiple bytes at once, such as copying a slice to the output,"] # [doc = " rather than iterating over one byte at a time."] # [inline] fn try_extend (& mut self , data : & [u8]) -> Result < () > { data . iter () . try_for_each (| d | self . try_push (* d)) } # [doc = " Push a single byte to be modified and/or stored."] fn try_push (& mut self , data : u8) -> Result < () > ; # [doc = " Finalize the serialization process."] fn finalize (self) -> Result < Self :: Output > ; }
};
}
