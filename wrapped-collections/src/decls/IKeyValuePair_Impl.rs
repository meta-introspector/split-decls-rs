macro_rules! IKeyValuePair_Impl {
    () => {
        pub trait IKeyValuePair_Impl < K , V > : windows_core :: IUnknownImpl where K : windows_core :: RuntimeType + 'static , V : windows_core :: RuntimeType + 'static , { fn Key (& self) -> windows_core :: Result < K > ; fn Value (& self) -> windows_core :: Result < V > ; }
    };
}

IKeyValuePair_Impl!()