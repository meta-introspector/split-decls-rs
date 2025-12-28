macro_rules! Outcome {
    () => {
        # [doc = " The positive result produced by [describe()][function::describe()]."] # [derive (Debug , Clone)] pub struct Outcome < 'name > { # [doc = " The name of the tag or branch that is closest to the commit `id`."] # [doc = ""] # [doc = " If `None`, no name was found but it was requested to provide the `id` itself as fallback."] pub name : Option < Cow < 'name , BStr > > , # [doc = " The input commit object id that we describe."] pub id : gix_hash :: ObjectId , # [doc = " The number of commits that are between the tag or branch with `name` and `id`."] # [doc = " These commits are all in the future of the named tag or branch."] pub depth : u32 , # [doc = " The mapping between object ids and their names initially provided by the describe call."] pub name_by_oid : HashMap < gix_hash :: ObjectId , Cow < 'name , BStr > > , # [doc = " The amount of commits we traversed."] pub commits_seen : u32 , }
    };
}

Outcome!();