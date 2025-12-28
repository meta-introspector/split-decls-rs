macro_rules! deps {
    () => {
        AssistContext!();
        Assists!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl Assists { pub (crate) fn new (ctx : & AssistContext < '_ > , resolve : AssistResolveStrategy) -> Assists { Assists { resolve , file : ctx . frange . file_id . file_id (ctx . db ()) , buf : Vec :: new () , allowed : ctx . config . allowed . clone () , } } pub (crate) fn finish (mut self) -> Vec < Assist > { self . buf . sort_by_key (| assist | assist . target . len ()) ; self . buf } pub (crate) fn add (& mut self , id : AssistId , label : impl Into < String > , target : TextRange , f : impl FnOnce (& mut SourceChangeBuilder) ,) -> Option < () > { let mut f = Some (f) ; self . add_impl (None , id , label . into () , target , & mut | it | f . take () . unwrap () (it)) } pub (crate) fn add_group (& mut self , group : & GroupLabel , id : AssistId , label : impl Into < String > , target : TextRange , f : impl FnOnce (& mut SourceChangeBuilder) ,) -> Option < () > { let mut f = Some (f) ; self . add_impl (Some (group) , id , label . into () , target , & mut | it | f . take () . unwrap () (it)) } fn add_impl (& mut self , group : Option < & GroupLabel > , id : AssistId , label : String , target : TextRange , f : & mut dyn FnMut (& mut SourceChangeBuilder) ,) -> Option < () > { if ! self . is_allowed (& id) { return None ; } let mut command = None ; let source_change = if self . resolve . should_resolve (& id) { let mut builder = SourceChangeBuilder :: new (self . file) ; f (& mut builder) ; command = builder . command . take () ; Some (builder . finish ()) } else { None } ; let label = Label :: new (label) ; let group = group . cloned () ; self . buf . push (Assist { id , label , group , target , source_change , command }) ; Some (()) } fn is_allowed (& self , id : & AssistId) -> bool { match & self . allowed { Some (allowed) => allowed . iter () . any (| kind | kind . contains (id . 1)) , None => true , } } }
    };
}

impl_6!()