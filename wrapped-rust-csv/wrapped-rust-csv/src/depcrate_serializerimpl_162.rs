// Generated macro for impl_162 (impl)
macro_rules! Depcrate_serializerimpl_162 {
() => {
// Module: crate::serializer
// Provides: {"impl_162"}
// Dependencies: {}
impl < 'w , W : io :: Write > SeHeader < 'w , W > { fn new (wtr : & 'w mut Writer < W >) -> Self { SeHeader { wtr , state : HeaderState :: Write } } fn wrote_header (& self) -> bool { use self :: HeaderState :: * ; match self . state { Write | ErrorIfWrite (_) => false , EncounteredStructField | InStructField => true , } } fn handle_scalar < T : fmt :: Display > (& mut self , name : T ,) -> Result < () , Error > { use self :: HeaderState :: * ; match self . state { Write => { self . state = ErrorIfWrite (error_scalar_outside_struct (name)) ; Ok (()) } ErrorIfWrite (_) | InStructField => Ok (()) , EncounteredStructField => Err (error_scalar_outside_struct (name)) , } } fn handle_container < T : fmt :: Display > (& mut self , name : T ,) -> Result < & mut Self , Error > { if let HeaderState :: InStructField = self . state { Err (error_container_inside_struct (name)) } else { Ok (self) } } }
};
}
