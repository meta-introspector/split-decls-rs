// Generated macro for derive_borrow_decode_inner (function)
macro_rules! Depcratederive_borrow_decode_inner {
() => {
// Module: crate
// Provides: {"derive_borrow_decode_inner"}
// Dependencies: {}
fn derive_borrow_decode_inner (input : TokenStream) -> Result < TokenStream > { let parse = Parse :: new (input) ? ; let (mut generator , attributes , body) = parse . into_generator () ; let attributes = attributes . get_attribute :: < ContainerAttributes > () ? . unwrap_or_default () ; match body { Body :: Struct (body) => { derive_struct :: DeriveStruct { fields : body . fields , attributes , } . generate_borrow_decode (& mut generator) ? ; } Body :: Enum (body) => { derive_enum :: DeriveEnum { variants : body . variants , attributes , } . generate_borrow_decode (& mut generator) ? ; } } generator . export_to_file ("bincode" , "BorrowDecode") ; generator . finish () }
};
}
