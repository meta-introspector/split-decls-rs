// Generated macro for create (function)
macro_rules! Depcrate_corpus_dbcreate {
() => {
// Module: crate::corpus::db
// Provides: {"create"}
// Dependencies: {}
pub fn create (path : impl AsRef < std :: path :: Path >) -> anyhow :: Result < rusqlite :: Connection > { let path = path . as_ref () ; let con = rusqlite :: Connection :: open (path) ? ; let meta_table = r#"
    CREATE TABLE if not exists meta(
        version int
    )"# ; con . execute_batch (meta_table) ? ; let version : Option < usize > = con . query_row ("SELECT version FROM meta" , [] , | r | r . get (0)) . optional () ? ; match version { None => { con . execute ("INSERT into meta(version) values(?)" , params ! [VERSION]) ? ; } Some (version) if version != VERSION => match con . close () { Ok (()) => { bail ! ("Cannot handle database with version {version}, cannot yet migrate to {VERSION} - maybe migrate by hand?") ; } Err ((_ , err)) => return Err (err . into ()) , } , _ => { } } con . execute_batch ("PRAGMA synchronous = OFF; PRAGMA journal_mode = WAL; PRAGMA wal_checkpoint(FULL); ") ? ; con . execute_batch (r#"
    CREATE TABLE if not exists runner(
        id integer PRIMARY KEY,
        vendor text,
        brand text,
        host_name text, -- this is just to help ID the runner
        UNIQUE (vendor, brand)
    )
    "# ,) ? ; con . execute_batch (r#"
    CREATE TABLE if not exists corpus(
        id integer PRIMARY KEY,
        root text UNIQUE -- the root path of all repositories we want to consider, as canonicalized path
    )
    "# ,) ? ; con . execute_batch (r"
    CREATE TABLE if not exists repository(
        id integer PRIMARY KEY,
        rela_path text, -- the path to the repository on disk, relative to the corpus root path, without leading `./` or `.\`
        corpus integer,
        odb_size integer, -- the object database size in bytes
        num_references integer, -- the total amount of references
        num_objects integer, -- the total amount of objects
        FOREIGN KEY (corpus) REFERENCES corpus (id)
        UNIQUE (rela_path, corpus)
    )
    " ,) ? ; con . execute_batch (r#"
    CREATE TABLE if not exists gitoxide_version(
        id integer PRIMARY KEY,
        version text UNIQUE -- the unique git version via gix describe
    )
    "# ,) ? ; con . execute_batch (r#"
    CREATE TABLE if not exists task(
        id integer PRIMARY KEY,
        short_name UNIQUE, -- the unique and permanent identifier for the task
        description text UNIQUE -- the descriptive name of the task, it can be changed at will
    )
    "# ,) ? ; con . execute_batch (r#"
    CREATE TABLE if not exists run(
        id integer PRIMARY KEY,
        repository integer,
        runner integer,
        task integer,
        gitoxide_version integer,
        insertion_time integer NOT NULL, -- in seconds since UNIX epoch
        duration real, -- in seconds or NULL if not yet finished (either successful or with failure)
        error text, -- or NULL if there was no error
        spans_json text, -- all spans collecteted while performing the run
        FOREIGN KEY (repository) REFERENCES repository (id),
        FOREIGN KEY (task) REFERENCES task (id),
        FOREIGN KEY (runner) REFERENCES runner (id),
        FOREIGN KEY (gitoxide_version) REFERENCES gitoxide_version (id)
    )
    "# ,) ? ; Ok (con) }
};
}
