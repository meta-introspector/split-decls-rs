// Generated macro for TABLE_INIT_SCRIPT (const)
macro_rules! Depcrate_reports_htmlTABLE_INIT_SCRIPT {
() => {
// Module: crate::reports::html
// Provides: {"TABLE_INIT_SCRIPT"}
// Dependencies: {}
const TABLE_INIT_SCRIPT : & str = r#"
<script type="text/javascript">
    window.addEventListener("load", (event) => {

        let prefers = window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
        let html = document.querySelector('html');

        html.classList.add(prefers);
        html.setAttribute('data-bs-theme', prefers);

        new DataTable('table.log-dancer-table',
        {
            paging: false,
            dom: '<"center" flpti  >'
        });

        let loading = document.getElementById("loading");
        loading.style.visibility = 'hidden';

        let tables = document.getElementById("tables");
        tables.style.visibility = 'visible';
    });
</script>
"# ;
};
}
