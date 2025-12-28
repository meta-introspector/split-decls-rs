macro_rules! deps {
    () => {
        Commit!();
        Id!();
        Read!();
        Item!();
        Info!();
        Note!();
        Error!();
    };
}

macro_rules! impl_860 {
    () => {
        deps!();
        # [doc = " Access"] impl < 'repo > Info < 'repo > { # [doc = " Provide an attached version of our [`id`][Info::id] field."] pub fn id (& self) -> crate :: Id < 'repo > { self . id . attach (self . repo) } # [doc = " Read the whole object from the object database."] # [doc = ""] # [doc = " Note that this is an expensive operation which shouldn't be performed unless one needs more than parent ids"] # [doc = " and commit time."] pub fn object (& self) -> Result < crate :: Commit < 'repo > , crate :: object :: find :: existing :: Error > { Ok (self . id () . object () ? . into_commit ()) } # [doc = " Provide an iterator yielding attached versions of our [`parent_ids`][Info::parent_ids] field."] pub fn parent_ids (& self) -> impl Iterator < Item = crate :: Id < 'repo > > + '_ { self . parent_ids . iter () . map (| id | id . attach (self . repo)) } # [doc = " Returns the commit-time of this commit."] # [doc = ""] # [doc = " ### Panics"] # [doc = ""] # [doc = " If the iteration wasn't ordered by date."] pub fn commit_time (& self) -> gix_date :: SecondsSinceUnixEpoch { self . commit_time . expect ("traversal involving date caused it to be set") } }
    };
}

impl_860!()