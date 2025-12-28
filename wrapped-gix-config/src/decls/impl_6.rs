macro_rules! deps {
    () => {
        Size!();
        SectionMut!();
        Section!();
        Event!();
        Whitespace!();
        Error!();
        Index!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl < 'a , 'event > SectionMut < 'a , 'event > { pub (crate) fn new (section : & 'a mut Section < 'event > , newline : SmallVec < u8 , 2 >) -> Self { let whitespace = Whitespace :: from_body (& section . body) ; Self { section , implicit_newline : true , whitespace , newline , } } pub (crate) fn get (& self , key : & ValueName < '_ > , start : Index , end : Index ,) -> Result < Cow < '_ , BStr > , lookup :: existing :: Error > { let mut expect_value = false ; let mut concatenated_value = BString :: default () ; for event in & self . section . 0 [start . 0 .. end . 0] { match event { Event :: SectionValueName (event_key) if event_key == key => expect_value = true , Event :: Value (v) if expect_value => return Ok (normalize_bstr (v . as_ref ())) , Event :: ValueNotDone (v) if expect_value => { concatenated_value . push_str (v . as_ref ()) ; } Event :: ValueDone (v) if expect_value => { concatenated_value . push_str (v . as_ref ()) ; return Ok (normalize_bstring (concatenated_value)) ; } _ => () , } } Err (lookup :: existing :: Error :: KeyMissing) } pub (crate) fn delete (& mut self , start : Index , end : Index) { self . section . body . 0 . drain (start . 0 .. end . 0) ; } pub (crate) fn set_internal (& mut self , index : Index , key : ValueName < 'event > , value : & BStr) -> Size { let mut size = 0 ; let body = & mut self . section . body . 0 ; body . insert (index . 0 , Event :: Value (escape_value (value) . into ())) ; size += 1 ; let sep_events = self . whitespace . key_value_separators () ; size += sep_events . len () ; body . splice (index . 0 .. index . 0 , sep_events . into_iter () . rev ()) . for_each (| _ | { }) ; body . insert (index . 0 , Event :: SectionValueName (key)) ; size += 1 ; Size (size) } # [doc = " Performs the removal, assuming the range is valid."] fn remove_internal (& mut self , range : Range < usize > , fix_whitespace : bool) -> Cow < 'event , BStr > { let events = & mut self . section . body . 0 ; if fix_whitespace && events . get (range . end) . is_some_and (| ev | matches ! (ev , Event :: Newline (_))) { events . remove (range . end) ; } let value = events . drain (range . clone ()) . fold (Cow :: Owned (BString :: default ()) , | mut acc : Cow < '_ , BStr > , e | { if let Event :: Value (v) | Event :: ValueNotDone (v) | Event :: ValueDone (v) = e { acc . to_mut () . extend (& * * v) ; } acc }) ; if fix_whitespace && range . start . checked_sub (1) . and_then (| pos | events . get (pos)) . is_some_and (| ev | matches ! (ev , Event :: Whitespace (_))) { events . remove (range . start - 1) ; } value } }
    };
}

impl_6!();