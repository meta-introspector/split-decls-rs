macro_rules! deps {
    () => {
        TimeZone!();
    };
}

macro_rules! Local {
    () => {
        deps!();
        # [doc = " The local timescale."] # [doc = ""] # [doc = " Using the [`TimeZone`](./trait.TimeZone.html) methods"] # [doc = " on the Local struct is the preferred way to construct `DateTime<Local>`"] # [doc = " instances."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use chrono::{DateTime, Local, TimeZone};"] # [doc = ""] # [doc = " let dt1: DateTime<Local> = Local::now();"] # [doc = " let dt2: DateTime<Local> = Local.timestamp_opt(0, 0).unwrap();"] # [doc = " assert!(dt1 >= dt2);"] # [doc = " ```"] # [derive (Copy , Clone , Debug)] # [cfg_attr (any (feature = "rkyv" , feature = "rkyv-16" , feature = "rkyv-32" , feature = "rkyv-64") , derive (Archive , Deserialize , Serialize) , archive (compare (PartialEq)) , archive_attr (derive (Clone , Copy , Debug)))] # [cfg_attr (feature = "rkyv-validation" , archive (check_bytes))] # [cfg_attr (feature = "arbitrary" , derive (arbitrary :: Arbitrary))] # [cfg_attr (feature = "defmt" , derive (defmt :: Format))] pub struct Local ;
    };
}

Local!();