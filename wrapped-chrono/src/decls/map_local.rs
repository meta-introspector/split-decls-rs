macro_rules! deps {
    () => {
        Utc!();
        NaiveDateTime!();
        TimeZone!();
        DateTime!();
    };
}

macro_rules! map_local {
    () => {
        deps!();
        # [doc = " Maps the local datetime to other datetime with given conversion function."] fn map_local < Tz : TimeZone , F > (dt : & DateTime < Tz > , mut f : F) -> Option < DateTime < Tz > > where F : FnMut (NaiveDateTime) -> Option < NaiveDateTime > , { f (dt . overflowing_naive_local ()) . and_then (| datetime | dt . timezone () . from_local_datetime (& datetime) . single ()) . filter (| dt | dt >= & DateTime :: < Utc > :: MIN_UTC && dt <= & DateTime :: < Utc > :: MAX_UTC) }
    };
}

map_local!()