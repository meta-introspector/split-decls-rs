macro_rules! IsoWeek {
    () => {
        # [doc = " ISO 8601 week."] # [doc = ""] # [doc = " This type, combined with [`Weekday`](../enum.Weekday.html),"] # [doc = " constitutes the ISO 8601 [week date](./struct.NaiveDate.html#week-date)."] # [doc = " One can retrieve this type from the existing [`Datelike`](../trait.Datelike.html) types"] # [doc = " via the [`Datelike::iso_week`](../trait.Datelike.html#tymethod.iso_week) method."] # [derive (PartialEq , Eq , PartialOrd , Ord , Copy , Clone , Hash)] # [cfg_attr (any (feature = "rkyv" , feature = "rkyv-16" , feature = "rkyv-32" , feature = "rkyv-64") , derive (Archive , Deserialize , Serialize) , archive (compare (PartialEq , PartialOrd)) , archive_attr (derive (Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Debug , Hash)))] # [cfg_attr (feature = "rkyv-validation" , archive (check_bytes))] pub struct IsoWeek { ywf : i32 , }
    };
}

IsoWeek!();