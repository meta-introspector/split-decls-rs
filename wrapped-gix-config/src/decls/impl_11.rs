macro_rules! deps {
    () => {
        Error!();
        Section!();
        SectionMut!();
        ValueMut!();
        Size!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl < 'borrow , 'event > ValueMut < 'borrow , '_ , 'event > { # [doc = " Returns the actual value. This is computed each time this is called"] # [doc = " requiring an allocation for multi-line values."] pub fn get (& self) -> Result < Cow < '_ , BStr > , lookup :: existing :: Error > { self . section . get (& self . key , self . index , self . index + self . size) } # [doc = " Update the value to the provided one. This modifies the value such that"] # [doc = " the Value event(s) are replaced with a single new event containing the"] # [doc = " new value."] pub fn set_string (& mut self , input : impl AsRef < str >) { self . set (input . as_ref ()) ; } # [doc = " Update the value to the provided one. This modifies the value such that"] # [doc = " the Value event(s) are replaced with a single new event containing the"] # [doc = " new value."] pub fn set < 'a > (& mut self , input : impl Into < & 'a BStr >) { if self . size . 0 > 0 { self . section . delete (self . index , self . index + self . size) ; } self . size = self . section . set_internal (self . index , self . key . to_owned () , input . into ()) ; } # [doc = " Removes the value. Does nothing when called multiple times in"] # [doc = " succession."] pub fn delete (& mut self) { if self . size . 0 > 0 { self . section . delete (self . index , self . index + self . size) ; self . size = Size (0) ; } } # [doc = " Return the section containing the value."] pub fn section (& self) -> & file :: Section < 'event > { & self . section } # [doc = " Convert this value into its owning mutable section."] pub fn into_section_mut (self) -> file :: SectionMut < 'borrow , 'event > { self . section } }
    };
}

impl_11!()