macro_rules! Slots {
    () => {
        # [doc = " Configures the number of slots in the index slotmap, which is fixed throughout the existence of the store."] # [derive (Copy , Clone , Debug)] pub enum Slots { # [doc = " The number of slots to use, that is the total number of indices we can hold at a time."] # [doc = " Using this has the advantage of avoiding an initial directory listing of the repository, and is recommended"] # [doc = " on the server side where the repository setup is controlled."] # [doc = ""] # [doc = " Note that this won't affect their packs, as each index can have one or more packs associated with it."] Given (u16) , # [doc = " Compute the number of slots needed, as probably best used on the client side where a variety of repositories is encountered."] AsNeededByDiskState { # [doc = " 1.0 means no safety, 1.1 means 10% more slots than needed"] multiplier : f32 , # [doc = " The minimum number of slots to assume"] minimum : usize , } , }
    };
}

Slots!();