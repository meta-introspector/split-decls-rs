macro_rules! deps {
    () => {
        FluentValue!();
        FluentArgs!();
        FluentNumberOptions!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        impl FluentNumberOptions { pub fn merge (& mut self , opts : & FluentArgs) { for (key , value) in opts . iter () { match (key , value) { ("type" , FluentValue :: String (n)) => { self . r#type = n . as_ref () . into () ; } ("style" , FluentValue :: String (n)) => { self . style = n . as_ref () . into () ; } ("currency" , FluentValue :: String (n)) => { self . currency = Some (n . to_string ()) ; } ("currencyDisplay" , FluentValue :: String (n)) => { self . currency_display = n . as_ref () . into () ; } ("useGrouping" , FluentValue :: String (n)) => { self . use_grouping = n != "false" ; } ("minimumIntegerDigits" , FluentValue :: Number (n)) => { self . minimum_integer_digits = Some (n . into ()) ; } ("minimumFractionDigits" , FluentValue :: Number (n)) => { self . minimum_fraction_digits = Some (n . into ()) ; } ("maximumFractionDigits" , FluentValue :: Number (n)) => { self . maximum_fraction_digits = Some (n . into ()) ; } ("minimumSignificantDigits" , FluentValue :: Number (n)) => { self . minimum_significant_digits = Some (n . into ()) ; } ("maximumSignificantDigits" , FluentValue :: Number (n)) => { self . maximum_significant_digits = Some (n . into ()) ; } _ => { } } } } }
    };
}

impl_77!()