macro_rules! deps {
    () => {
        Create!();
        WalkerIter!();
    };
}

macro_rules! Walker {
    () => {
        deps!();
        # [doc = " A walker is a traversal state, but where part of the traversal"] # [doc = " information is supplied manually to each next call."] # [doc = ""] # [doc = " This for example allows graph traversals that don't hold a borrow of the"] # [doc = " graph they are traversing."] pub trait Walker < Context > { type Item ; # [doc = " Advance to the next item"] fn walk_next (& mut self , context : Context) -> Option < Self :: Item > ; # [doc = " Create an iterator out of the walker and given `context`."] fn iter (self , context : Context) -> WalkerIter < Self , Context > where Self : Sized , Context : Clone , { WalkerIter { walker : self , context , } } }
    };
}

Walker!();