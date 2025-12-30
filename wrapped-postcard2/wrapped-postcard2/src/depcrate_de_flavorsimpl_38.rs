// Generated macro for impl_38 (impl)
macro_rules! Depcrate_de_flavorsimpl_38 {
() => {
// Module: crate::de::flavors
// Provides: {"impl_38"}
// Dependencies: {}
impl < 'de > Flavor < 'de > for Slice < 'de > { type Remainder = & 'de [u8] ; type Source = & 'de [u8] ; # [inline] fn pop (& mut self) -> Result < u8 > { if self . cursor == self . end { Err (Error :: DeserializeUnexpectedEnd) } else { unsafe { let res = Ok (* self . cursor) ; self . cursor = self . cursor . add (1) ; res } } } # [inline] fn size_hint (& self) -> Option < usize > { Some ((self . end as usize) - (self . cursor as usize)) } # [inline] fn try_take_n (& mut self , ct : usize) -> Result < & 'de [u8] > { let remain = (self . end as usize) - (self . cursor as usize) ; if remain < ct { Err (Error :: DeserializeUnexpectedEnd) } else { unsafe { let sli = core :: slice :: from_raw_parts (self . cursor , ct) ; self . cursor = self . cursor . add (ct) ; Ok (sli) } } } # [doc = " Return the remaining (unused) bytes in the Deserializer"] fn finalize (self) -> Result < & 'de [u8] > { let remain = (self . end as usize) - (self . cursor as usize) ; unsafe { Ok (core :: slice :: from_raw_parts (self . cursor , remain)) } } }
};
}
