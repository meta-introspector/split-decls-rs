// Generated macro for impl_27 (impl)
macro_rules! Depcrate_safe_bagimpl_27 {
() => {
// Module: crate::safe_bag
// Provides: {"impl_27"}
// Dependencies: {}
impl :: der :: EncodeValue for SafeBag { fn value_len (& self) -> :: der :: Result < :: der :: Length > { let content = AnyRef :: from_der (& self . bag_value) ? ; use :: der :: Encode as _ ; [self . bag_id . encoded_len () ? , :: der :: asn1 :: ContextSpecificRef { tag_number : :: der :: TagNumber (0) , tag_mode : :: der :: TagMode :: Explicit , value : & content , } . encoded_len () ? , self . bag_attributes . encoded_len () ? ,] . into_iter () . try_fold (:: der :: Length :: ZERO , | acc , len | acc + len) } fn encode_value (& self , writer : & mut impl :: der :: Writer) -> :: der :: Result < () > { use :: der :: Encode as _ ; self . bag_id . encode (writer) ? ; let content = AnyRef :: from_der (& self . bag_value) ? ; :: der :: asn1 :: ContextSpecificRef { tag_number : :: der :: TagNumber (0) , tag_mode : :: der :: TagMode :: Explicit , value : & content , } . encode (writer) ? ; self . bag_attributes . encode (writer) ? ; Ok (()) } }
};
}
