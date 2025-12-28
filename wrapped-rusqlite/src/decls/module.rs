macro_rules! deps {
    () => {
        Module!();
    };
}

macro_rules! module {
    () => {
        deps!();
        macro_rules ! module { ($ lt : lifetime , $ vt : ty , $ ct : ty , $ xc : expr , $ xd : expr , $ xu : expr) => { & Module { base : ffi :: sqlite3_module { iVersion : 2 , xCreate : $ xc , xConnect : Some (rust_connect ::<$ vt >) , xBestIndex : Some (rust_best_index ::<$ vt >) , xDisconnect : Some (rust_disconnect ::<$ vt >) , xDestroy : $ xd , xOpen : Some (rust_open ::<$ vt >) , xClose : Some (rust_close ::<$ ct >) , xFilter : Some (rust_filter ::<$ ct >) , xNext : Some (rust_next ::<$ ct >) , xEof : Some (rust_eof ::<$ ct >) , xColumn : Some (rust_column ::<$ ct >) , xRowid : Some (rust_rowid ::<$ ct >) , xUpdate : $ xu , xBegin : None , xSync : None , xCommit : None , xRollback : None , xFindFunction : None , xRename : None , xSavepoint : None , xRelease : None , xRollbackTo : None , .. ZERO_MODULE } , phantom : PhantomData ::<&$ lt $ vt >, } } ; }
    };
}

module!();