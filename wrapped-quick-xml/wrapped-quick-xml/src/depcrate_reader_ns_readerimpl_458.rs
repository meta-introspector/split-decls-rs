// Generated macro for impl_458 (impl)
macro_rules! Depcrate_reader_ns_readerimpl_458 {
() => {
// Module: crate::reader::ns_reader
// Provides: {"impl_458"}
// Dependencies: {}
# [doc = " Private methods"] impl < R > NsReader < R > { # [inline] fn new (reader : Reader < R >) -> Self { Self { reader , ns_resolver : NamespaceResolver :: default () , pending_pop : false , } } fn read_event_impl < 'i , B > (& mut self , buf : B) -> Result < Event < 'i > > where R : XmlSource < 'i , B > , { self . pop () ; let event = self . reader . read_event_impl (buf) ; self . process_event (event) } pub (super) fn pop (& mut self) { if self . pending_pop { self . ns_resolver . pop () ; self . pending_pop = false ; } } pub (super) fn process_event < 'i > (& mut self , event : Result < Event < 'i > >) -> Result < Event < 'i > > { match event { Ok (Event :: Start (e)) => { self . ns_resolver . push (& e) ? ; Ok (Event :: Start (e)) } Ok (Event :: Empty (e)) => { self . ns_resolver . push (& e) ? ; self . pending_pop = true ; Ok (Event :: Empty (e)) } Ok (Event :: End (e)) => { self . pending_pop = true ; Ok (Event :: End (e)) } e => e , } } }
};
}
