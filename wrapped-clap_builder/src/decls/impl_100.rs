macro_rules! deps {
    () => {
        AnyValueId!();
        Extensions!();
        Extension!();
        AnyValue!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        impl Extensions { # [allow (dead_code)] pub (crate) fn get < T : Extension > (& self) -> Option < & T > { let id = AnyValueId :: of :: < T > () ; self . extensions . get (& id) . map (| e | { e . downcast_ref :: < T > () . expect ("`Extensions` tracks values by type") }) } # [allow (dead_code)] pub (crate) fn set < T : Extension > (& mut self , tagged : T) -> bool { let value = AnyValue :: new (tagged) ; let id = value . type_id () ; self . extensions . insert (id , value) . is_some () } # [allow (dead_code)] pub (crate) fn remove < T : Extension > (& mut self) -> Option < T > { let id = AnyValueId :: of :: < T > () ; self . extensions . remove (& id) . map (| e | { e . downcast_into :: < T > () . expect ("`Extensions` tracks values by type") }) } pub (crate) fn update (& mut self , other : & Self) { for (key , value) in other . extensions . iter () { self . extensions . insert (* key , value . clone ()) ; } } }
    };
}

impl_100!();