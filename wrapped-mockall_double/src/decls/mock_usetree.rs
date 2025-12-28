macro_rules! mock_usetree {
    () => {
        fn mock_usetree (mut orig : & mut UseTree) { match & mut orig { UseTree :: Glob (star) => { compile_error (star . span () , "Cannot double glob imports.  Import by fully qualified name instead.") ; } , UseTree :: Group (ug) => { for ut in ug . items . iter_mut () { mock_usetree (ut) ; } } , UseTree :: Name (un) => { * orig = UseTree :: Rename (UseRename { ident : mock_ident (& un . ident) , as_token : < Token ! [as] > :: default () , rename : un . ident . clone () }) ; } , UseTree :: Path (up) => { mock_usetree (up . tree . as_mut ()) ; } , UseTree :: Rename (ur) => { ur . ident = mock_ident (& ur . ident) } , } }
    };
}

mock_usetree!()