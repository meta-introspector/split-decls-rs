macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        # [doc = " Constructors indicating what kind of mapping is created."] # [doc = ""] # [doc = " Only these combinations of values are valid."] impl < 'a > Entry < 'a > { # [doc = " An entry that changes the name by an email."] pub fn change_name_by_email (proper_name : impl Into < & 'a BStr > , commit_email : impl Into < & 'a BStr >) -> Self { Entry { new_name : Some (proper_name . into ()) , old_email : commit_email . into () , .. Default :: default () } } # [doc = " An entry that changes the email by an email."] pub fn change_email_by_email (proper_email : impl Into < & 'a BStr > , commit_email : impl Into < & 'a BStr >) -> Self { Entry { new_email : Some (proper_email . into ()) , old_email : commit_email . into () , .. Default :: default () } } # [doc = " An entry that changes the email by a name and email."] pub fn change_email_by_name_and_email (proper_email : impl Into < & 'a BStr > , commit_name : impl Into < & 'a BStr > , commit_email : impl Into < & 'a BStr > ,) -> Self { Entry { new_email : Some (proper_email . into ()) , old_email : commit_email . into () , old_name : Some (commit_name . into ()) , .. Default :: default () } } # [doc = " An entry that changes a name and the email by an email."] pub fn change_name_and_email_by_email (proper_name : impl Into < & 'a BStr > , proper_email : impl Into < & 'a BStr > , commit_email : impl Into < & 'a BStr > ,) -> Self { Entry { new_name : Some (proper_name . into ()) , new_email : Some (proper_email . into ()) , old_email : commit_email . into () , .. Default :: default () } } # [doc = " An entry that changes a name and email by a name and email."] pub fn change_name_and_email_by_name_and_email (proper_name : impl Into < & 'a BStr > , proper_email : impl Into < & 'a BStr > , commit_name : impl Into < & 'a BStr > , commit_email : impl Into < & 'a BStr > ,) -> Self { Entry { new_name : Some (proper_name . into ()) , new_email : Some (proper_email . into ()) , old_name : Some (commit_name . into ()) , old_email : commit_email . into () , } } }
    };
}

impl_10!();