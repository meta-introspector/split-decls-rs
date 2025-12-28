macro_rules! deps {
    () => {
        EmailEntry!();
    };
}

macro_rules! Snapshot {
    () => {
        deps!();
        # [doc = " A data-structure to efficiently store a list of entries for optimal, case-insensitive lookup by email and"] # [doc = " optionally name to find mappings to new names and/or emails."] # [doc = ""] # [doc = " The memory layout is efficient, even though lots of small allocations are performed to store strings of emails and names."] # [doc = ""] # [doc = " ### Handling of invalid `SignatureRef::time`"] # [doc = ""] # [doc = " As the `time` field in [`SignatureRef`](gix_actor::SignatureRef) as passed by the caller maybe invalid,"] # [doc = " something that should be very rare but is possible, we decided to not expose this fallibility in the API."] # [doc = " Hence, the user may separately check for the correctness of `time`, which we replace with [`gix_date::Time::default()`]"] # [doc = " in case of parse errors."] # [derive (Default , Clone , Debug , Eq , PartialEq)] pub struct Snapshot { # [doc = " Sorted by `old_email`"] entries_by_old_email : Vec < snapshot :: EmailEntry > , }
    };
}

Snapshot!()