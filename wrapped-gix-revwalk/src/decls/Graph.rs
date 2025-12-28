macro_rules! deps {
    () => {
        IdMap!();
    };
}

macro_rules! Graph {
    () => {
        deps!();
        # [doc = " A graph of commits which additionally allows to associate data with commits."] # [doc = ""] # [doc = " It starts empty, but each access may fill it with commit information."] # [doc = " Note that the traversal can be accelerated if a [commit-graph][gix_commitgraph::Graph] is also made available."] # [doc = ""] # [doc = " ### About replacements"] # [doc = ""] # [doc = " Object replacements is an object database feature to substitute one object with another. We assume that this is transparently"] # [doc = " implemented by the `find` function that returns objects. Also we assume that the commitgraph as been written with replacements"] # [doc = " active to provide a consistent view."] # [doc = ""] # [doc = " ### Odb or `find` configuration"] # [doc = ""] # [doc = " The `find` handle should be setup to *quickly determine if an object exists or not* to assure quick operation *on shallow repositories*."] # [doc = " This typically means that it should not re-read the odb if there is an object miss."] # [doc = ""] # [doc = " Most usage of the Graph will benefit from fast ODB lookups, so setting up an object cache will be beneficial. If that's not the case,"] # [doc = " the method docs will inform about that."] # [doc = ""] # [doc = " Additionally, and only if `T` is [`Commit<T>`][graph::Commit], there is *no need for an object cache* as we keep track of"] # [doc = " everything related to commit traversal in our own hashmap."] pub struct Graph < 'find , 'cache , T > { # [doc = " A way to resolve a commit from the object database."] find : Box < dyn gix_object :: Find + 'find > , # [doc = " A way to speedup commit access, essentially a multi-file commit database."] cache : Option < & 'cache gix_commitgraph :: Graph > , # [doc = " The set of cached commits that we have seen once, along with data associated with them."] map : graph :: IdMap < T > , # [doc = " A buffer for writing commit data into."] buf : Vec < u8 > , # [doc = " Another buffer we typically use to store parents."] parent_buf : Vec < u8 > , }
    };
}

Graph!();