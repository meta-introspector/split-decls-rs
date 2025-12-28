macro_rules! deps {
    () => {
        Body!();
        Event!();
    };
}

macro_rules! impl_78 {
    () => {
        deps!();
        impl Body < '_ > { pub (crate) fn as_ref (& self) -> & [Event < '_ >] { & self . 0 } # [doc = " Returns the range containing the value events for the `value_name`, with value range being `None` if there is"] # [doc = " no key-value separator and only a 'fake' Value event with an empty string in side."] # [doc = " If the value is not found, `None` is returned."] pub (crate) fn key_and_value_range_by (& self , value_name : & ValueName < '_ > ,) -> Option < (Range < usize > , Option < Range < usize > >) > { let mut value_range = Range :: default () ; let mut key_start = None ; for (i , e) in self . 0 . iter () . enumerate () . rev () { match e { Event :: SectionValueName (k) => { if k == value_name { key_start = Some (i) ; break ; } value_range = Range :: default () ; } Event :: Value (_) => { (value_range . start , value_range . end) = (i , i) ; } Event :: ValueNotDone (_) | Event :: ValueDone (_) => { if value_range . end == 0 { value_range . end = i ; } else { value_range . start = i ; } } _ => () , } } key_start . map (| key_start | { # [allow (clippy :: range_plus_one)] let value_range = value_range . start .. value_range . end + 1 ; let key_range = key_start .. value_range . end ; (key_range , (value_range . start != key_start + 1) . then_some (value_range)) }) } }
    };
}

impl_78!()