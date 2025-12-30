// Generated macro for Message (struct)
macro_rules! Depcrate_messageMessage {
() => {
// Module: crate::message
// Provides: {"Message"}
// Dependencies: {}
# [derive (Clone , Debug)] pub struct Message < 'a > { msg_type : u8 , flags : u8 , serial : Option < NonZeroU32 > , path : Option < Cow < 'a , strings :: ObjectPath > > , interface : Option < Cow < 'a , strings :: InterfaceName > > , member : Option < Cow < 'a , strings :: MemberName > > , error_name : Option < Cow < 'a , strings :: ErrorName > > , reply_serial : Option < NonZeroU32 > , destination : Option < Cow < 'a , strings :: BusName > > , sender : Option < Cow < 'a , strings :: BusName > > , signature : Option < Cow < 'a , strings :: SignatureMulti > > , body : Cow < 'a , [u8] > , is_big_endian : bool , }
};
}
