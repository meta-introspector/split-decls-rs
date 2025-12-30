// Generated macro for impl_73 (impl)
macro_rules! Depcrateimpl_73 {
() => {
// Module: crate
// Provides: {"impl_73"}
// Dependencies: {}
impl < 'sval , Ok > StreamEnum < 'sval > for Unsupported < Ok > { type Ok = Ok ; type Tuple = Self ; type Record = Self ; type Nested = Self ; fn tag (self , _ : Option < sval :: Tag > , _ : Option < sval :: Label > , _ : Option < sval :: Index > ,) -> Result < Self :: Ok > { Err (Error :: invalid_value ("enums are unsupported")) } fn tagged_computed < V : sval :: Value > (self , _ : Option < sval :: Tag > , _ : Option < sval :: Label > , _ : Option < sval :: Index > , _ : V ,) -> Result < Self :: Ok > { Err (Error :: invalid_value ("enums are unsupported")) } fn tuple_begin (self , _ : Option < sval :: Tag > , _ : Option < sval :: Label > , _ : Option < sval :: Index > , _ : Option < usize > ,) -> Result < Self :: Record > { Err (Error :: invalid_value ("enums are unsupported")) } fn record_begin (self , _ : Option < sval :: Tag > , _ : Option < sval :: Label > , _ : Option < sval :: Index > , _ : Option < usize > ,) -> Result < Self :: Record > { Err (Error :: invalid_value ("enums are unsupported")) } fn nested < F : FnOnce (Self :: Nested) -> Result < < Self :: Nested as StreamEnum < 'sval > > :: Ok > > (self , _ : Option < sval :: Tag > , _ : Option < sval :: Label > , _ : Option < sval :: Index > , _ : F ,) -> Result < Self :: Ok > { Err (Error :: invalid_value ("enums are unsupported")) } fn empty (self) -> Result < Self :: Ok > { Err (Error :: invalid_value ("enums are unsupported")) } }
};
}
