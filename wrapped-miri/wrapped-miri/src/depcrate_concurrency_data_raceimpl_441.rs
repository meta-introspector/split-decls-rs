// Generated macro for impl_441 (impl)
macro_rules! Depcrate_concurrency_data_raceimpl_441 {
() => {
// Module: crate::concurrency::data_race
// Provides: {"impl_441"}
// Dependencies: {}
impl AccessType { fn description (self , ty : Option < Ty < '_ > > , size : Option < Size >) -> String { let mut msg = String :: new () ; if let Some (size) = size { if size == Size :: ZERO { assert ! (self == AccessType :: AtomicLoad) ; assert ! (ty . is_none ()) ; return format ! ("multiple differently-sized atomic loads, including one load") ; } msg . push_str (& format ! ("{}-byte {}" , size . bytes () , msg)) } msg . push_str (match self { AccessType :: NaRead (w) => w . description () , AccessType :: NaWrite (w) => w . description () , AccessType :: AtomicLoad => "atomic load" , AccessType :: AtomicStore => "atomic store" , AccessType :: AtomicRmw => "atomic read-modify-write" , }) ; if let Some (ty) = ty { msg . push_str (& format ! (" of type `{ty}`")) ; } msg } fn is_atomic (self) -> bool { match self { AccessType :: AtomicLoad | AccessType :: AtomicStore | AccessType :: AtomicRmw => true , AccessType :: NaRead (_) | AccessType :: NaWrite (_) => false , } } fn is_read (self) -> bool { match self { AccessType :: AtomicLoad | AccessType :: NaRead (_) => true , AccessType :: NaWrite (_) | AccessType :: AtomicStore | AccessType :: AtomicRmw => false , } } fn is_retag (self) -> bool { matches ! (self , AccessType :: NaRead (NaReadType :: Retag) | AccessType :: NaWrite (NaWriteType :: Retag)) } }
};
}
