macro_rules! deps {
    () => {
        Span!();
        Level!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl Span { # [doc = " Create a new span."] # [doc = ""] # [doc = " This constructor is typically invoked by the [`crate::span!`] macro."] pub fn new (level : crate :: Level , meta : & 'static Metadata < 'static > , values : & tracing_core :: field :: ValueSet < '_ > ,) -> Self { if level > crate :: MAX_LEVEL { Self { id : None } } else { with_dispatcher (| dispatch | { let id = dispatch . new_span (& tracing_core :: span :: Attributes :: new (meta , values)) ; dispatch . enter (& id) ; Self { id : Some ((id , dispatch . clone () , meta)) , } }) } } # [doc = " Record a single `field` to take `value`."] # [doc = ""] # [doc = " ### Panics"] # [doc = ""] # [doc = " If the field name wasn't mentioned when the span was created."] pub fn record < V > (& self , field : & str , value : V) -> & Self where V : field :: Value , { if let Some ((_ , _ , meta)) = & self . id { let fields = meta . fields () ; let field = fields . field (field) . unwrap_or_else (| | panic ! ("Field name '{field}' must be registered at creation time.")) ; self . record_all (& fields . value_set (& [(& field , Some (& value as & dyn field :: Value))])) ; } self } fn record_all (& self , values : & field :: ValueSet < '_ >) -> & Self { if let Some ((id , dispatch , _)) = & self . id { let record = span :: Record :: new (values) ; dispatch . record (id , & record) ; } self } }
    };
}

impl_4!();