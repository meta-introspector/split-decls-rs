macro_rules! deps {
    () => {
        Entry!();
        FluentFunction!();
        GetEntry!();
        FluentBundle!();
        FluentResource!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl < R : Borrow < FluentResource > , M > GetEntry for FluentBundle < R , M > { fn get_entry_message (& self , id : & str) -> Option < & ast :: Message < & str > > { self . entries . get (id) . and_then (| ref entry | match entry { Entry :: Message ((resource_idx , entry_idx)) => { let res = self . resources . get (* resource_idx) ? . borrow () ; if let ast :: Entry :: Message (msg) = res . get_entry (* entry_idx) ? { Some (msg) } else { None } } _ => None , }) } fn get_entry_term (& self , id : & str) -> Option < & ast :: Term < & str > > { self . entries . get (id) . and_then (| ref entry | match entry { Entry :: Term ((resource_idx , entry_idx)) => { let res = self . resources . get (* resource_idx) ? . borrow () ; if let ast :: Entry :: Term (msg) = res . get_entry (* entry_idx) ? { Some (msg) } else { None } } _ => None , }) } fn get_entry_function (& self , id : & str) -> Option < & FluentFunction > { self . entries . get (id) . and_then (| ref entry | match entry { Entry :: Function (function) => Some (function) , _ => None , }) } }
    };
}

impl_23!()