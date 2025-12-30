// Generated macro for py_to_datetime_with_microseconds (function)
macro_rules! Depcrate_x509_commonpy_to_datetime_with_microseconds {
() => {
// Module: crate::x509::common
// Provides: {"py_to_datetime_with_microseconds"}
// Dependencies: {}
pub (crate) fn py_to_datetime_with_microseconds (py : pyo3 :: Python < '_ > , val : pyo3 :: Bound < '_ , pyo3 :: types :: PyDateTime > ,) -> pyo3 :: PyResult < (asn1 :: DateTime , Option < u32 >) > { let val_utc = if val . get_tzinfo () . is_none () { val . into_any () } else { let utc = pyo3 :: types :: PyTzInfo :: utc (py) ? ; val . call_method1 (pyo3 :: intern ! (py , "astimezone") , (utc ,)) ? } ; let datetime = asn1 :: DateTime :: new (val_utc . getattr (pyo3 :: intern ! (py , "year")) ? . extract () ? , val_utc . getattr (pyo3 :: intern ! (py , "month")) ? . extract () ? , val_utc . getattr (pyo3 :: intern ! (py , "day")) ? . extract () ? , val_utc . getattr (pyo3 :: intern ! (py , "hour")) ? . extract () ? , val_utc . getattr (pyo3 :: intern ! (py , "minute")) ? . extract () ? , val_utc . getattr (pyo3 :: intern ! (py , "second")) ? . extract () ? ,) . unwrap () ; let microseconds : u32 = val_utc . getattr (pyo3 :: intern ! (py , "microsecond")) ? . extract () ? ; let microseconds = if microseconds > 0 { Some (microseconds) } else { None } ; Ok ((datetime , microseconds)) }
};
}
