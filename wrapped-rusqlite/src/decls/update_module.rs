macro_rules! deps {
    () => {
        UpdateVTab!();
        Module!();
        VTabKind!();
    };
}

macro_rules! update_module {
    () => {
        deps!();
        # [doc = " Create a modifiable virtual table implementation."] # [doc = ""] # [doc = " Step 2 of [Creating New Virtual Table Implementations](https://sqlite.org/vtab.html#creating_new_virtual_table_implementations)."] # [must_use] pub fn update_module < 'vtab , T : UpdateVTab < 'vtab > > () -> & 'static Module < 'vtab , T > { match T :: KIND { VTabKind :: EponymousOnly => { module ! ('vtab , T , T :: Cursor , None , None , Some (rust_update ::< T >)) } VTabKind :: Eponymous => { module ! ('vtab , T , T :: Cursor , Some (rust_connect ::< T >) , Some (rust_disconnect ::< T >) , Some (rust_update ::< T >)) } _ => { module ! ('vtab , T , T :: Cursor , Some (rust_create ::< T >) , Some (rust_destroy ::< T >) , Some (rust_update ::< T >)) } } }
    };
}

update_module!();