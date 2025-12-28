macro_rules! deps {
    () => {
        Namespace!();
        Error!();
        PartialNameRef!();
    };
}

macro_rules! expand {
    () => {
        deps!();
        # [doc = " Given a `namespace` 'foo we output 'refs/namespaces/foo', and given 'foo/bar' we output 'refs/namespaces/foo/refs/namespaces/bar'."] # [doc = ""] # [doc = " For more information, consult the [git namespace documentation](https://git-scm.com/docs/gitnamespaces)."] pub fn expand < 'a , Name , E > (namespace : Name) -> Result < Namespace , gix_validate :: reference :: name :: Error > where Name : TryInto < & 'a PartialNameRef , Error = E > , gix_validate :: reference :: name :: Error : From < E > , { let namespace = & namespace . try_into () ? . 0 ; let mut out = BString :: default () ; for component in namespace . split_str (b"/") { out . push_str ("refs/namespaces/") ; out . push_str (component) ; out . push_str (b"/") ; } Ok (Namespace (out)) }
    };
}

expand!();