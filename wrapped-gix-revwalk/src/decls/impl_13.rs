macro_rules! deps {
    () => {
        Graph!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        # [doc = " Initialization"] impl < 'find , 'cache , T > Graph < 'find , 'cache , T > { # [doc = " Create a new instance with `objects` to retrieve commits and optionally `cache` to accelerate commit access."] # [doc = ""] # [doc = " ### Performance"] # [doc = ""] # [doc = " `find` should be optimized to access the same object repeatedly, ideally with an object cache to keep the last couple of"] # [doc = " most recently used commits."] # [doc = " Furthermore, **none-existing commits should not trigger the pack-db to be refreshed.** Otherwise, performance may be sub-optimal"] # [doc = " in shallow repositories as running into non-existing commits will trigger a refresh of the `packs` directory."] pub fn new (objects : impl gix_object :: Find + 'find , cache : Option < & 'cache gix_commitgraph :: Graph >) -> Self { Graph { find : Box :: new (objects) , cache , map : gix_hashtable :: HashMap :: default () , buf : Vec :: new () , parent_buf : Vec :: new () , } } }
    };
}

impl_13!()